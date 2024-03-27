export declare type InnerMethod<Args, Result> = (state: Record<string, any>, args: Args) => Result | Promise<Result>;

export declare type MethodOutput<Result> = {
    state: Record<string, any>;
    result: Result;
};

export function defineMethod<Args, Result>(method: InnerMethod<Args, Result>) {
    return async (state: Record<string, any>, args: Args) => {
        const result = await method(state, args);
        return {
            state: state,
            result,
        };
    };
}
