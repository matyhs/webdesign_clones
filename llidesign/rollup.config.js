import postcss from 'rollup-plugin-postcss';
import copy from 'rollup-plugin-copy';

export default [{
    input: {
        bundle: './wasm.js'
    },
    output: {
        dir: './dist/js',
        format: 'iife'
    }
},
{
    input: {
        style: './static/styles.css'
    },
    output: {
        dir: './dist/css'
    },
    plugins: [
        postcss({
            extract: true,
            plugins: []
        }),
        copy({
            targets: [
                { src: './static/index.html', dest: './dist' }
            ]
        })
    ]
}]