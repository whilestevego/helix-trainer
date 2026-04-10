import { resolve, join } from "path";
import { getExerciseFiles } from "./verify";

async function restoreFromTemplate(
  filePath: string,
  exercisesDir: string,
  templateDir: string,
): Promise<boolean> {
  // Compute the relative path within exercises/
  const relPath = filePath.replace(exercisesDir + "/", "");
  const templatePath = join(templateDir, relPath);

  const templateFile = Bun.file(templatePath);
  if (!(await templateFile.exists())) {
    console.error(`\x1b[31m✗\x1b[0m No template found for ${relPath}`);
    return false;
  }

  await Bun.write(filePath, templateFile);
  console.log(`\x1b[32m✓\x1b[0m Reset ${relPath}`);
  return true;
}

export async function resetExercise(
  filePath: string,
  templateDir: string,
): Promise<void> {
  // Determine exercises dir from the file path
  const parts = filePath.split("/");
  const exIdx = parts.lastIndexOf("exercises");
  if (exIdx < 0) {
    console.error("File path must be inside an exercises/ directory.");
    process.exit(1);
  }
  const exercisesDir = parts.slice(0, exIdx + 1).join("/");
  await restoreFromTemplate(filePath, exercisesDir, templateDir);
}

export async function resetAll(
  exercisesDir: string,
  templateDir: string,
): Promise<void> {
  const files = await getExerciseFiles(exercisesDir);

  if (files.length === 0) {
    console.log("No .hxt exercise files found.");
    return;
  }

  let count = 0;
  for (const file of files) {
    if (await restoreFromTemplate(file, exercisesDir, templateDir)) count++;
  }

  console.log(`\nReset ${count}/${files.length} exercises.`);
}
