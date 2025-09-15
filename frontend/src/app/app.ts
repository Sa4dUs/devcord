import { Component } from "@angular/core";
import { RouterModule } from "@angular/router";
import { ErrorService } from "./core/services/error-service";

@Component({
    selector: "app-root",
    standalone: true,
    imports: [RouterModule],
    templateUrl: "./app.html",
    styleUrls: ["./app.scss"],
})
export class App {
    protected title = "frontend";

    constructor(private errorsMap: ErrorService) {}
}
