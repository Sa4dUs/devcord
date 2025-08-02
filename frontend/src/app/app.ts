import { Component } from "@angular/core";
import { RouterModule } from "@angular/router";
import { ErrorsHandling } from "./errors/errors";

@Component({
    selector: "app-root",
    standalone: true,
    imports: [RouterModule],
    templateUrl: "./app.html",
    styleUrls: ["./app.scss"],
})
export class App {
    protected title = "frontend";

    constructor(private errorsMap: ErrorsHandling) {}
}
