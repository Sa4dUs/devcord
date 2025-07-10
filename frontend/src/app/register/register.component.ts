import { Component } from '@angular/core';
import {  FormBuilder, FormGroup, Validators } from '@angular/forms';
import { ReactiveFormsModule } from '@angular/forms';


// Decorador mágico que le dice a Angular: "Ey, esto es un componente"
//Stalone seguirá siendo un misterio 
@Component({
  selector: 'app-register',
  standalone: true,
  imports: [ReactiveFormsModule],
  templateUrl: './register.component.html',
  styleUrls: ['./register.component.scss']
})
export class RegisterComponent {
  registerForm: FormGroup; // Aquí guardamos el formulario y sus movidas

  constructor(private fb: FormBuilder) {
    // Creamos el formulario:
    this.registerForm = this.fb.group({
      username: ['', Validators.required], // Campo obligatorio
      email: ['', [Validators.required, Validators.email]], 
      password: ['', Validators.required], 
      telephone: [''] //opcional
    });
  }

  // Función que se lanza cuando pulsas el botón de "Registrarse"
  onSubmit(): void {
    if (this.registerForm.valid) {
      console.log('Datos del formulario:', this.registerForm.value);
    } else {
      // Algo va mal, no seas vago y rellena lo que falta
      console.warn('Por la gloria del Imperio Mongol, rellena el formulario!');
    }
  }
}
