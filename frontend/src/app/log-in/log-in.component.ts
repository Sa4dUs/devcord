import { Component, inject, signal } from "@angular/core";
import {
    ReactiveFormsModule,
    Validators,
    FormGroup,
    FormControl,
} from "@angular/forms";
import { Router } from "@angular/router";
import { LoginService } from "../services/login.service";

@Component({
    selector: "log-in",
    imports: [ReactiveFormsModule],
    templateUrl: "./log-in.component.html",
    styleUrls: ["./log-in.component.scss"],
})
export class LogInComponent {
    router = inject(Router);
    loginService = inject(LoginService);
    loginForm = new FormGroup({
        username: new FormControl("", Validators.required),
        password: new FormControl("", Validators.required),
    });
    error = signal<string | null>(null);

    onSubmit() {
        let { username, password } = this.loginForm.value;
        this.loginService
            .login(username ?? "", password ?? "")
            .then(({ token, username, user_id }) => {
                this.setError("");
                localStorage.setItem(
                    "user",
                    JSON.stringify({
                        username,
                        user_id,
                    }),
                );
                localStorage.setItem("token", token);

                this.router.navigate(["main-menu"]);
            })
            .catch(({ status }) => {
                this.setError(`${status}`);
            });
    }

    setError(error: string | null) {
        this.error.set(error);
    }
}
