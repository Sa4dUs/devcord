import { Component, OnInit, inject } from "@angular/core";
import { RouterModule } from "@angular/router";
import { isPlatformBrowser } from "@angular/common";
import { PLATFORM_ID } from "@angular/core";
import { LogoutComponent } from "../../logout/logout.component";

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

    private platformId = inject(PLATFORM_ID);

    ngOnInit(): void {
        if (isPlatformBrowser(this.platformId)) {
            const storedUser = localStorage.getItem("user");
            if (storedUser) {
                this.user = JSON.parse(storedUser);
            }
        }
    }
}
