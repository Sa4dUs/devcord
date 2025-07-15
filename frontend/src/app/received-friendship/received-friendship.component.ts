import { Component, OnInit } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { RouterModule } from '@angular/router';

interface FriendRequest {
  from_user_username: string;
  state: 'pending' | 'accepted' | 'rejected';
  created_at: string;
}

@Component({
  selector: 'app-received-friendship',
  standalone: true,
  imports: [CommonModule, FormsModule, RouterModule],
  templateUrl: './received-friendship.component.html',
  styleUrls: ['./received-friendship.component.scss']
})
export class FriendRequestsComponent implements OnInit {
  friendRequests: FriendRequest[] = [];
  isLoading = false;
  error: string | null = null;

  pageSize = 10;
  currentPage = 0;

  constructor(private http: HttpClient) {}

  ngOnInit(): void {
    this.loadFriendRequests();
  }

  loadFriendRequests(): void {
    const token = localStorage.getItem('token');
    if (!token) {
      console.error('No token found.');
      return;
    }

    const from = this.currentPage * this.pageSize;

    const params = {
      from: from.toString(),
      to: this.pageSize.toString()
    };

    this.isLoading = true;
    this.error = null;

    this.http.get<FriendRequest[]>('http://lamoara.duckdns.org:6969/api/user/friendship/received', {
      headers: { Authorization: `Bearer ${token}` },
      params
    }).subscribe({
      next: (data) => {

        this.friendRequests = data;
        this.isLoading = false;
      },
      error: (err) => {
        this.error = 'Error al cargar solicitudes.';
        this.isLoading = false;
        console.error(err); 
      }
    });
  }

  nextPage(): void {
    this.currentPage++;
    this.loadFriendRequests();
  }

  prevPage(): void {
    if (this.currentPage > 0) {
      this.currentPage--;
      this.loadFriendRequests();
    }
  }
  //tengo que actualizar a la fuerza desde aquí porque sino pasa lo que pasa...cargando...
  //¿cómo se hace? lo sabremos en próximos capítulos. Posible solución 5J=> actualizar por todas partes aunque no haga falta.
onClickAccept(request: FriendRequest): void {
  const token = localStorage.getItem('token');
  if (!token) {
    console.error('No token found.');
    return;
  }

  const body = {
    to_user_username: request.from_user_username
  };

  this.http.post('http://lamoara.duckdns.org:6969/api/user/friendship/accept', body, {
    headers: { Authorization: `Bearer ${token}` }
  }).subscribe({
    next: () => {
      console.log('Solicitud aceptada correctamente.');
      this.loadFriendRequests(); // recarga la lista
    },
    error: (err) => {
      if (err.status === 404) {
        console.warn('No se encontró la solicitud.');
      } else if (err.status === 409) {
        console.warn('Ya sois amigos.');
      } else {
        console.error('Error al aceptar solicitud:', err);
      }
    }
  });
}


onClickReject(request: FriendRequest): void {
const token = localStorage.getItem('token');
  if (!token) {
    console.error('No token found.');
    return;
  }

  const body = {
    to_user_username: request.from_user_username
  };

  this.http.post('http://lamoara.duckdns.org:6969/api/user/friendship/reject', body, {
    headers: { Authorization: `Bearer ${token}` }
  }).subscribe({
    next: () => {
      console.log('Morirás solo.');
      this.loadFriendRequests(); 
    },
    error: (err) => {
      if (err.status === 404) {
        console.warn('No se encontró la solicitud.');
      } else if (err.status === 409) {
        console.warn('Ya sois amigos.');
      } else {
        console.error('Error al aceptar solicitud:', err);
      }
    }
  });
}

}
