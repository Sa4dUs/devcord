import { Component } from "@angular/core";
import { ActivatedRoute } from "@angular/router";

@Component({
    selector: "group",
    standalone: true,
    templateUrl: "./group.component.html",
    styleUrls: ["./group.component.scss"],
})
export class GroupComponent {
    groupId!: string;

    constructor(private route: ActivatedRoute) {
        this.groupId = this.route.snapshot.paramMap.get("groupId")!;
    }
}
