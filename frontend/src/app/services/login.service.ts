import { Injectable } from "@angular/core";
import { SERVER_ROUTE } from "../../environment/environment.secret";

interface LoginResponse {
    token: string;
    user_id: string;
    username: string;
}

@Injectable({
    providedIn: "root",
})
export class LoginService {
    async login(username: string, password: string): Promise<LoginResponse> {
        const res = await fetch(`${SERVER_ROUTE}/api/auth/login`, {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify({
                username,
                password,
            }),
        });

        if (!res.ok) return Promise.reject({ status: res.status });

        const data: LoginResponse = await res.json();
        return data;
    }
}
