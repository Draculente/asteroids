import { GameRessource } from '../bindings/GameRessource';
const API_URL = process.env.API_URL;
const ERROR_TIMEOUT = 5000;

enum Method {
    GET = "GET",
    POST = "POST",
    PUT = "PUT",
    DELETE = "DELETE"
}

interface UserNameResponse {
    username: string
}

interface LoginResponse {
    access_token: string
}

interface ApiError {
    short: string | null,
    error: string
}

type Optional<T> = T | undefined | null;

export default class Api {
    loading: boolean = false;
    private error: ApiError | null = null;
    private errorTimeout: string = "";
    private token_field: string = "";
    private username_field: string = "";
    private onLoginCallbacks: (() => void)[] = [];
    validatingToken: boolean = false;

    constructor() {
        const token = localStorage.getItem("token") ?? "";
        this.validateToken(token);
    }

    registerOnLogin(callback: () => void) {
        this.onLoginCallbacks.push(callback);
        if (this.loggedIn) {
            callback();
        }
    }

    set token(token: Optional<string>) {
        this.token_field = token ?? "";
        if (token) {
            localStorage.setItem("token", token);
        } else {
            localStorage.removeItem("token");
        }
    }

    get token(): string {
        return this.token_field;
    }

    get errorMessage(): string | undefined {
        return this.error?.short || this.error?.error;
    }

    private set errorMessage(value: string | undefined) {
        if (!value) return;
        console.error(value);
        this.error = {
            short: null,
            error: value
        };
        clearTimeout(this.errorTimeout);
        this.errorTimeout = setTimeout(() => {
            this.error = null;
        }, ERROR_TIMEOUT) as any;
    }

    get loggedIn(): boolean {
        return this.token !== "";
    }

    get username(): string {
        return this.loggedIn ? this.username_field : "";
    }

    async login(username: string, password: string): Promise<string> {
        const response = await this.makeRequest<LoginResponse>("/user/login", Method.POST, {
            username,
            password
        });
        if (response?.access_token) {
            this.handleLogin(response.access_token, username);
        }
        return this.token;
    }

    async register(username: string, password: string): Promise<void> {
        try {
            let res = await this.makeRequest<{}>("/user/", Method.POST, {
                username,
                password
            });
            if (res !== null)
                await this.login(username, password);
        } catch (error) {
            // Nothing to do here
        }
    }

    async deleteAccount(): Promise<void> {
        await this.makeRequest<{}>("/user/", Method.DELETE, undefined, this.token);
        this.logout();
    }

    private async validateToken(token: string): Promise<boolean> {
        try {
            this.validatingToken = true;
            if (!token) return false;

            const response = await this.makeRequest<UserNameResponse>("/user/", Method.GET, undefined, token, true);
            if (response?.username) {
                this.handleLogin(token, response.username);
                return true;
            } else {
                this.logout();
            }
            return false;
        } catch (error) {
            return false;
        } finally {
            this.validatingToken = false;
        }
    }

    async createGame(gameRessource: GameRessource): Promise<Optional<number>> {
        if (!this.loggedIn) return;
        const game = await this.makeRequest<GameRessource>("/games/", Method.POST, gameRessource, this.token);
        return game?.id;
    }

    async saveGame(gameRessource: GameRessource): Promise<void> {
        if (!this.loggedIn) return;
        if (!gameRessource.id) return;
        await this.makeRequest<GameRessource>(`/games/${gameRessource.id}`, Method.PUT, gameRessource, this.token);
    }

    async setGameEnded(gameId: number): Promise<void> {
        await this.makeRequest<{}>(`/games/${gameId}`, Method.PUT, { ended: true }, this.token);
    }

    async loadGame(): Promise<Optional<GameRessource>> {
        const response = await this.makeRequest<GameRessource[]>("/games/?latest=true", Method.GET, undefined, this.token);
        if (response?.length) {
            return response[0];
        }
    }

    logout() {
        this.token = "";
        this.username_field = "";
        localStorage.removeItem("token");
    }

    private handleLogin(token: string, username: string) {
        this.token = token;
        this.username_field = username;
        this.onLoginCallbacks.forEach(callback => callback());
    }

    private async makeRequest<T>(endpoint: String, method: Method, body?: any, token?: String, expectError: boolean = false): Promise<Optional<T>> {
        try {
            this.loading = true;
            const response = await fetch(`${API_URL}${endpoint}`, {
                method,
                headers: {
                    "Content-Type": "application/json",
                    "Authorization": token ? `Bearer ${token}` : ""
                },
                body: body ? JSON.stringify(body) : undefined
            });
            const json = await response.json();
            if (json.error) {
                throw json.error;
            }
            return json as T;
        } catch (error: any) {
            if (!expectError) {
                this.errorMessage = error?.message || error;
            }
            return null;
        } finally {
            this.loading = false;
        }
    }
}