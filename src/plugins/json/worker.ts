import { defineMethod } from '~/plugins/_base';

export const onPaste = defineMethod(async (_state, args: {}) => {
    console.log('onPaste', args);
    return {
        time: new Date().getTime(),
    };
});

export const prettier = defineMethod(async (_state, args) => {
    return JSON.stringify(args, null, 2);
});
