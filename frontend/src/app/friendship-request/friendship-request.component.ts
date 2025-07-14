import { Component } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { FormsModule } from '@angular/forms';
import { RouterModule, Router } from '@angular/router';

@Component({
  selector: 'app-friendship-request',
  standalone: true,
  imports: [FormsModule, RouterModule],
  templateUrl: './friendship-request.component.html',
  styleUrls: ['./friendship-request.component.scss']
})
export class FriendshipRequestComponent {
  constructor(private http: HttpClient, private router: Router) {}

  Request = {
    to_user_username: ''
  };

sendRequest() {
  if (!this.Request.to_user_username) {
    console.warn('Sino tienes amigos no envies solicitudes de amistad');
    return;
  }
  

  const token = localStorage.getItem('token');
//aquí debug ya hecho, no es problema de que no se almacene el token
  if (!token) {
    console.error('No hay token. No puedes enviar solicitudes sin estar logueado.');
    return;
  }

  this.http.post<any>('http://lamoara.duckdns.org:6969/api/user/friendship/request',
    {
      to_user_username: this.Request.to_user_username
    },
    {
      headers: { //eh aquí un no problema 
        Authorization: `Bearer ${token}`
      }
    }
  ).subscribe({
    next: (data) => {
      console.log('Solicitud enviada con éxito:', data);
      this.Request.to_user_username = '';
      this.router.navigate(['/user']);
    },
    error: (error) => {
      switch (error.status) {
        case 409:
          console.warn('Nombre de usuario erróneo.');
          break;
        default:
          console.error('Error desconocido:', error);
      }
    }
  });
}

}
