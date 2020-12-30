import init, {run} from './dist/js/llidesign.js';

async function exec() {
    await init('/js/llidesign_bg.wasm');
    run();
}

exec();