const { exec } = require('child_process');
const fs = require('fs');
const path = require('path');

// Run wasm-pack build --target web
exec('wasm-pack build --target web', (error, stdout, stderr) => {
  if (error) {
    console.error(`Error executing wasm-pack: ${error}`);
    return;
  }
  console.log(stdout);

  // Copy the .wasm file
  const source = './pkg/wasm_bind_gen_block_on_bg.wasm';
  const destination = './vite-project/public/wasm_bind_gen_block_on_bg.wasm';

  fs.copyFile(source, destination, (err) => {
    if (err) {
      console.error(`Error copying file: ${err}`);
      return;
    }
    console.log('File copied successfully.');

    // Change to vite-project directory
    process.chdir(path.join(__dirname, 'vite-project'));

    // Run npm install
    exec('npm install', (installError, installStdout, installStderr) => {
      if (installError) {
        console.error(`Error executing npm install: ${installError}`);
        return;
      }
      console.log(installStdout);
    });
  });
});