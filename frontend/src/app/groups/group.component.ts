import { Component } from "@angular/core";
import { ActivatedRoute } from "@angular/router";
import { MemberListComponent } from "./member-list/member-list.component";
import { DeleteUserComponent } from "./delete-user/delete-user.component";

@Component({
    selector: "group",
    standalone: true,
    imports: [MemberListComponent, DeleteUserComponent],
    templateUrl: "./group.component.html",
    styleUrls: ["./group.component.scss"],
})
export class GroupComponent {
    members: { userId: string }[] = [];
    groupId!: string;
    membersLoaded = false;

    constructor(private route: ActivatedRoute) {
        this.groupId = this.route.snapshot.paramMap.get("groupId")!;
    }

    activateMembersLoaded(members: { userId: string }[]) {
        this.members = members;
        this.membersLoaded = true;
    }
}
