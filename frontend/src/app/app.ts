import { Component } from "@angular/core";
import { RouterOutlet } from "@angular/router";
import { RegisterComponent } from "./register/register.component";

@Component({
  selector: "app-root",
  imports: [RouterOutlet, RegisterComponent],
  templateUrl: "./app.html",
  styleUrl: "./app.scss",
})
export class App {
  protected title = "frontend";
}
