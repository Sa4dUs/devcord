import { Component } from "@angular/core";
import { WebcamModule } from 'ngx-webcam';
import { Call } from "./call/call.component";  

@Component({
    selector: "app-root",
    standalone: true,
    imports: [WebcamModule, Call],
    templateUrl: "./app.html",
    styleUrls: ["./app.scss"],
})
export class App {
    protected title = "frontend";
}
