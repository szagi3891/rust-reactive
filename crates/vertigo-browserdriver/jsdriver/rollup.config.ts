import tsPlugin from '@rollup/plugin-typescript';

export default {
    input: './src/jsdriver.ts',
    output: {
        file: './out/jsdriver.js',
        format: 'es'
    },
    plugins: [
        tsPlugin(),
    ]
};
