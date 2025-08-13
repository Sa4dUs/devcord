import { Routes } from "@angular/router";
import { LogInComponent } from "./features/auth/login/login.component";
import { RegisterComponent } from "./features/auth/register/register.component";
import { HomeComponent } from "./features/home/home.component";
import { DashboardComponent } from "./features/dashboard/dashboard.component";
import { UserComponent } from "./features/user/user.component";
import { FriendshipRequestComponent } from "./features/user/friendship/friendship-request/friendship-request.component";
import { FriendRequestsComponent } from "./features/user/friendship/received-friendship/received-friendship.component";
import { FriendshipSentComponent } from "./features/user/friendship/friendship-sent/friendship-sent.component";
import { FriendshipFriendComponent } from "./features/user/friendship/friend/friend.component";
import { BlockComponent } from "./features/user/blocking/block/block.component";
import { UnblockComponent } from "./features/user/blocking/unblock/unblock.component";
import { UpdateUsernameComponent } from "./features/user/update/update.component";

export const routes: Routes = [
    { path: "", redirectTo: "home", pathMatch: "full" },

    { path: "login", component: LogInComponent },
    { path: "register", component: RegisterComponent },

    { path: "home", component: HomeComponent },
    { path: "dashboard", component: DashboardComponent },

    {
        path: "user",
        component: UserComponent,
        children: [
            {
                path: "friendship/request",
                component: FriendshipRequestComponent,
            },
            { path: "friendship/received", component: FriendRequestsComponent },
            { path: "friendship/sent", component: FriendshipSentComponent },
            { path: "friendship/friend", component: FriendshipFriendComponent },
            { path: "blocking/block", component: BlockComponent },
            { path: "blocking/unblock", component: UnblockComponent },
            { path: "update", component: UpdateUsernameComponent },
        ],
    },
];
