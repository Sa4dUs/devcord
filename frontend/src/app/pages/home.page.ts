import { Component } from "@angular/core";
import { RouterModule } from "@angular/router";

@Component({
    selector: "app-home",
    standalone: true,
    imports: [RouterModule],
    templateUrl: "./home.page.html",
    styleUrls: ["./home.page.scss"],
})
export class HomePage {}
