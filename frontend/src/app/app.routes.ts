import { Routes } from '@angular/router';
import { RegisterComponent } from './register/register.component';
import { LogInComponent } from './logIn/logIn.component';
import { HomeComponent } from './home/home.component';
import { UserComponent } from './user/user.component';
import { FriendshipRequestComponent } from './friendship-request/friendship-request.component';
import { FriendRequestsComponent } from './recieved-friendship/received-friendship.component';

export const routes: Routes = [
  { path: 'register', component: RegisterComponent },
  { path: 'login', component: LogInComponent },
  { path: 'home', component: HomeComponent },
  { path: 'user', component: UserComponent },
  { path: 'friendship-request', component: FriendshipRequestComponent},
  {path: 'recieved-friendship', component: FriendRequestsComponent},
  { path: '', redirectTo: 'home', pathMatch: 'full' } 
];
