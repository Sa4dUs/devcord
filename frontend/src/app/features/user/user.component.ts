import { RouterModule } from "@angular/router";
import { isPlatformBrowser } from "@angular/common";
import { Component, inject, OnInit, PLATFORM_ID } from "@angular/core";
import { LogoutComponent } from "../auth/logout/logout.component";

@Component({
    selector: "app-user",
    standalone: true,
    imports: [RouterModule, LogoutComponent],
    templateUrl: "./user.component.html",
    styleUrls: ["./user.component.scss"],
})
export class UserComponent implements OnInit {
    user = {
        username: "",
        email: "",
        telephone: "",
    };

    private readonly platformId = inject(PLATFORM_ID);

    ngOnInit(): void {
        if (isPlatformBrowser(this.platformId)) {
            const storedUser = localStorage.getItem("user");
            if (storedUser) {
                this.user = JSON.parse(storedUser);
            }
        }
    }
}
