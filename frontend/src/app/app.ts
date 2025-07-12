import { Component } from "@angular/core";
import { RouterOutlet } from "@angular/router";
import { RegisterComponent } from "./register/register.component";
import { LogInComponent } from "./logIn/logIn.component";

@Component({
  selector: "app-root",
  imports: [RouterOutlet, RegisterComponent,LogInComponent],
  templateUrl: "./app.html",
  styleUrl: "./app.scss",
})
export class App {
  protected title = "frontend";
}
