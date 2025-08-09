import { Component } from "@angular/core";
import { ActivatedRoute } from "@angular/router";
import { MemberListComponent } from "./member-list/member-list.component";

@Component({
    selector: "group",
    standalone: true,
    imports: [MemberListComponent],
    templateUrl: "./group.component.html",
    styleUrls: ["./group.component.scss"],
})
export class GroupComponent {
    groupId!: string;

    constructor(private route: ActivatedRoute) {
        this.groupId = this.route.snapshot.paramMap.get("groupId")!;
    }
}
