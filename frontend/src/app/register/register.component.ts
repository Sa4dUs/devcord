import { Component, inject } from "@angular/core";
import { ReactiveFormsModule, FormBuilder, Validators } from "@angular/forms";
import { HttpClient } from "@angular/common/http";
import { Router } from "@angular/router";
import { RegisterResponseType } from "../../types/Auth";
import { ErrorsHandling } from "../errors/errors";
import { SERVER_ROUTE } from "../../environment/environment.secret";

const context = "register";

@Component({
    selector: "app-register",
    standalone: true,
    imports: [ReactiveFormsModule],
    templateUrl: "./register.component.html",
    styleUrls: ["./register.component.scss"],
})
export class RegisterComponent {
    private fb = inject(FormBuilder);
    private http = inject(HttpClient);
    private router = inject(Router);

    constructor(private errorsMap: ErrorsHandling) {}

    registerForm = this.fb.group({
        username: ["", Validators.required],
        email: ["", [Validators.required, Validators.email]],
        password: ["", Validators.required],
        telephone: [""],
    });

    onSubmit(): void {
        if (!this.registerForm.valid) {
            // This could be more clarifier
            console.warn("Fill the form correctly");
            return;
        }

        const { username, email, password, telephone } =
            this.registerForm.value;

        this.http
            .post<RegisterResponseType>(SERVER_ROUTE + "/api/auth/register", {
                username,
                email,
                password,
                telephone,
            })
            .subscribe({
                next: (data) => {
                    localStorage.setItem(
                        "user",
                        JSON.stringify({
                            username,
                            email,
                            telephone,
                            user_id: data.user_id,
                        }),
                    );

                    if (data.token) {
                        localStorage.setItem("token", data.token);
                    }

                    console.log("Register succesfully:", data);
                    this.router.navigate(["/user"]);
                },
                error: (error) => {
                    console.error(
                        this.errorsMap.getErrorMessage(context, error),
                    );
                },
            });

        console.log("Data sent:", this.registerForm.value);
    }
}
