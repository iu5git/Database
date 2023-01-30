const { invoke } = window.__TAURI__.tauri;

export async function createUser(login, password) {
    const user = await invoke('create_user', { login, password });
    return user;
}

export async function addUser(user) {
    try {
        let user_id = await invoke('add_user', { user });
        return user_id;
    } catch (error) {
        console.error(error);
        return null;
    }
}

export async function tryLoginUser(login, password) {
    const maybeUserPromise = invoke('try_login_user', { login, password });
    return maybeUserPromise;
}
