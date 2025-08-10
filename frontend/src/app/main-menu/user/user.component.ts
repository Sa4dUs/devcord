import { Component, OnInit, ViewChild, inject } from "@angular/core";
import { RouterModule } from "@angular/router";
import { isPlatformBrowser } from "@angular/common";
import { PLATFORM_ID } from "@angular/core";
import { LogOutComponent } from "../../logout/logOut.component"; //seguro que lo de la ruta es motivo de caos...pero...

@Component({
    selector: "app-user",
    standalone: true,
    imports: [RouterModule, LogOutComponent],
    templateUrl: "./user.component.html",
    styleUrls: ["./user.component.scss"],
})
export class UserComponent implements OnInit {
    user = {
        username: "",
        email: "",
        telephone: "",
    };

    @ViewChild(LogOutComponent) logoutComp!: LogOutComponent; // para poder llamar a openDialog()

    private platformId = inject(PLATFORM_ID);

    ngOnInit(): void {
        if (isPlatformBrowser(this.platformId)) {
            const storedUser = localStorage.getItem("user");
            if (storedUser) {
                this.user = JSON.parse(storedUser);
            }
        }
    }

    openLogoutDialog() {
        this.logoutComp.openDialog();
    }
}
