import init, {run} from './pkg/llidesign.js';

async function exec() {
    await init('/pkg/llidesign_bg.wasm');
    run();
}

exec();