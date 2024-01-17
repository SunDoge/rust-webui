


type Result<T> = {
    t: "Ok" | "Err",
    c: T | string
}



export async function webuiCall<T>(func: string, input: any): Promise<T> {
    let resp = await webui.call(func, JSON.stringify(input));
    let result: Result<T> = JSON.parse(resp);
    if (result.t === 'Ok') {
        return result.c as T;
    } else {
        throw Error(result.c as string);
    }
}


