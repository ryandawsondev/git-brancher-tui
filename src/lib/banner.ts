// Prints the ASCII art banner with tool name, version, and description.

const BLUE = "\x1b[0;94m";
const RESET = "\x1b[0m";

const ASCII_ART = `
    ____  ____  ___    _   __________  ____________ 
   / __ )/ __ \/   |  / | / / ____/ / / / ____/ __ \
  / __  / /_/ / /| | /  |/ / /   / /_/ / __/ / /_/ /
 / /_/ / _, _/ ___ |/ /|  / /___/ __  / /___/ _, _/ 
/_____/_/ |_/_/  |_/_/ |_/\____/_/ /_/_____/_/ |_|       
`;

export function printBanner(
  name: string,
  version: string,
  description: string,
): void {
  process.stdout.write(BLUE + ASCII_ART + RESET + "\n");
  console.log(
    "=====================================================================",
  );
  console.log(`${name} (v${version})`);
  console.log(description);
  console.log(
    "=====================================================================",
  );
  console.log();
}
