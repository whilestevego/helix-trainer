#!/usr/bin/env bun

import { resolve, dirname, join } from "path";
import { verify, verifyAll, getExerciseFiles, parseHxt } from "../src/verify";
import { showProgress } from "../src/progress";
import { resetExercise, resetAll } from "../src/reset";

const PACKAGE_ROOT = resolve(dirname(new URL(import.meta.url).pathname), "..");
const TEMPLATE_DIR = resolve(PACKAGE_ROOT, "exercises");

const [command, ...args] = process.argv.slice(2);

const USAGE = `
helix-trainer — Interactive Helix keybinding exercises for Zed

Usage:
  helix-trainer init [dir]       Generate exercise project (default: ./helix-exercises)
  helix-trainer verify [file]    Check one exercise, or all if no file given
  helix-trainer progress         Show completion stats per module
  helix-trainer reset [file]     Reset one exercise, or all if no file given
  helix-trainer next             Show the next incomplete exercise

Getting started:
  helix-trainer init
  cd helix-exercises
  # Open this folder in Zed, start with 01-movement/01-basic-motion.hxt

All commands except 'init' operate on ./exercises/ in the current directory.
`.trim();

function findExercisesDir(): string {
  const cwd = process.cwd();
  // Check if CWD has exercises/ directly
  const direct = resolve(cwd, "exercises");
  // Or if CWD IS the exercises directory
  if (cwd.endsWith("/exercises")) return cwd;
  return direct;
}

async function copyDir(src: string, dest: string): Promise<number> {
  const { Glob } = await import("bun");
  const glob = new Glob("**/*");
  let count = 0;

  for await (const relPath of glob.scan({ cwd: src })) {
    const srcPath = join(src, relPath);
    const destPath = join(dest, relPath);

    const srcFile = Bun.file(srcPath);
    // Skip if it's a directory indicator (size 0 with no content)
    const stat = await srcFile.exists();
    if (!stat) continue;

    // Ensure parent directory exists
    const destDir = dirname(destPath);
    await Bun.spawn(["mkdir", "-p", destDir]).exited;

    // Copy file
    await Bun.write(destPath, srcFile);
    count++;
  }

  return count;
}

async function init(targetArg?: string): Promise<void> {
  const target = resolve(targetArg || "helix-exercises");

  // Check if target already exists with exercises
  const existingExercises = join(target, "exercises");
  const hasExisting = await Bun.file(join(existingExercises, "README.md")).exists();

  if (hasExisting) {
    console.error(`\x1b[33m!\x1b[0m ${target} already contains exercises.`);
    console.error(`  Use 'helix-trainer reset-all' from that directory to restore them.`);
    process.exit(1);
  }

  console.log(`\n  Generating Helix training exercises...\n`);

  // Copy exercises/ into target/exercises/
  const exercisesDest = join(target, "exercises");
  const count = await copyDir(TEMPLATE_DIR, exercisesDest);

  // Create a minimal .gitignore
  await Bun.write(join(target, ".gitignore"), "*.db\n.DS_Store\n");

  console.log(`  \x1b[32m✓\x1b[0m Created ${count} exercise files in ${target}/exercises/`);
  console.log(`
  Next steps:
    cd ${targetArg || "helix-exercises"}
    # Open this folder in Zed as a workspace
    # Start with exercises/01-movement/01-basic-motion.hxt

  Commands (run from inside the project):
    helix-trainer progress       Show your completion stats
    helix-trainer verify         Check all exercises
    helix-trainer next           See the next incomplete exercise
    helix-trainer reset          Reset all exercises to original
`);
}

async function findNext(exercisesDir: string): Promise<void> {
  const files = await getExerciseFiles(exercisesDir);

  for (const file of files) {
    const result = await parseHxt(file);
    if (!result.passed) {
      const rel = file.replace(resolve(process.cwd()) + "/", "");
      console.log(rel);
      return;
    }
  }
  console.log("All exercises completed!");
}

switch (command) {
  case "init": {
    await init(args[0]);
    break;
  }
  case "verify": {
    const file = args[0];
    if (file) {
      const result = await verify(resolve(file));
      process.exit(result.passed ? 0 : 1);
    } else {
      const allPassed = await verifyAll(findExercisesDir());
      process.exit(allPassed ? 0 : 1);
    }
    break;
  }
  case "progress": {
    await showProgress(findExercisesDir());
    break;
  }
  case "reset": {
    const file = args[0];
    if (file) {
      await resetExercise(resolve(file), TEMPLATE_DIR);
    } else {
      await resetAll(findExercisesDir(), TEMPLATE_DIR);
    }
    break;
  }
  case "next": {
    await findNext(findExercisesDir());
    break;
  }
  default: {
    console.log(USAGE);
    process.exit(command ? 1 : 0);
  }
}
