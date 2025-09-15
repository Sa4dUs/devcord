import { Component } from "@angular/core";
import { ActivatedRoute } from "@angular/router";
import { MemberListComponent } from "../components/member-list/member-list.component";
import { AddUserComponent } from "../components/add-user/add-user.component";
import { CommonModule } from "@angular/common";
import { CallButtonComponent } from "../components/call-button/call-button.component";
import { RemoveUserComponent } from "../components/remove-user/remove-user.component";

//TODO: @AlexGarciaPrada make it dialog
@Component({
    selector: "group",
    imports: [
        MemberListComponent,
        RemoveUserComponent,
        AddUserComponent,
        CommonModule,
        CallButtonComponent,
    ],
    templateUrl: "./group.page.html",
    styleUrls: ["./group.page.scss"],
})
export class GroupPage {
    members: string[] = [];
    groupId!: string;
    membersLoaded = false;

    constructor(private route: ActivatedRoute) {
        this.groupId = this.route.snapshot.paramMap.get("groupId")!;
    }

    activateMembersLoaded(members: string[]) {
        this.members = members;
        this.membersLoaded = true;
    }
}
