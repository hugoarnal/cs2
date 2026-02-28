use anyhow::Result;
use std::process::{Child, Command, Output, Stdio};

// The reason why patches are directly in the code is because I want
// anyone to just download cs2 and use patches without the need for
// external files.
//
// Patches content go after this line (with description if possible)

// This patch allows for any Python 3.x version above 3.11 to be ran.
pub(crate) const EPICLANG_PYTHON_VERSION: &str = "
From 6069feda756042771fe72fae1e8d1d7162c22a4e Mon Sep 17 00:00:00 2001
From: Hugo ARNAL <hugo@hugoarnal.com>
Date: Sat, 28 Feb 2026 21:44:58 +0100
Subject: [PATCH] ref: verify python version according to `python3 --version`

---
 epiclang | 15 ++++++++++++++-
 1 file changed, 14 insertions(+), 1 deletion(-)

diff --git a/epiclang b/epiclang
index 5445c8e..4823fc9 100755
--- a/epiclang
+++ b/epiclang
@@ -1,6 +1,9 @@
 #!/usr/bin/env bash
 # This script ensures that epiclang is run with at least Python 3.11

+LOWEST_MAJOR=3
+LOWEST_MINOR=11
+
 PYTHON_VERSIONS=(python3.13 python3.12 python3.11)

 PYTHON_CMD=\"\"
@@ -11,8 +14,18 @@ for version in \"${PYTHON_VERSIONS[@]}\"; do
     fi
 done

+if [ -z \"$PYTHON_CMD\" ] && command -v \"python3\" >/dev/null 2>&1; then
+    PYTHON_VERSION=$(python3 --version | sed \"s/Python //\")
+    MAJOR=$(echo \"$PYTHON_VERSION\" | cut -d '.' -f 1)
+    MINOR=$(echo \"$PYTHON_VERSION\" | cut -d '.' -f 2)
+
+    if [[ $MAJOR -eq $LOWEST_MAJOR && $MINOR -ge $LOWEST_MINOR ]]; then
+        PYTHON_CMD=\"python3\"
+    fi
+fi
+
 if [ -z \"$PYTHON_CMD\" ]; then
-    echo \"Error: No suitable Python version found (${PYTHON_VERSIONS[*]})\" >&2
+    echo \"Error: No suitable Python version found (must be at least $LOWEST_MAJOR.$LOWEST_MINOR)\" >&2
     exit 1
 fi

--
2.53.0
";

fn echo_patch(next_command: &mut Child, patch: &str) -> Result<Output> {
    Ok(Command::new("echo")
        .args([patch])
        .stdout(next_command.stdin.take().unwrap())
        .spawn()?
        .wait_with_output()?)
}

#[allow(dead_code)]
pub fn apply_patch(file: &str, patch: &str) -> Result<bool> {
    let mut dryrun_patch = Command::new("patch")
        .args(["-R", "-p0", "-s", "-f", "--dry-run", file])
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .spawn()?;
    let _ = echo_patch(&mut dryrun_patch, patch);

    if !dryrun_patch.wait_with_output()?.status.success() {
        let mut actual_patch = Command::new("sudo")
            .args(["patch", "-p0", "-s", "-f", file])
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .spawn()?;
        let _ = echo_patch(&mut actual_patch, patch);
        return Ok(actual_patch.wait()?.success());
    }
    Ok(false)
}
