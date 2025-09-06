import { Component, inject, Input } from "@angular/core";
import { Router } from "@angular/router";

@Component({
    selector: "call-button",
    imports: [],
    templateUrl: "./call-button.component.html",
    styleUrl: "./call-button.component.scss",
})
export class CallButtonComponent {
    @Input() groupId: string | undefined;

    private router = inject(Router);

    startCall() {
        this.router.navigate(["group/" + this.groupId + "/call"]);
    }
}
