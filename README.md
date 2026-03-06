📋 Lista de Tareas en Solana (Rust + Anchor)

Este proyecto implementa un programa en la blockchain de Solana utilizando Rust y el framework Anchor.
El programa permite crear y gestionar una lista de tareas (To-Do List) almacenada en la blockchain, donde cada usuario puede:

Crear su propia lista
Agregar tareas
Eliminar tareas
Ver tareas
Marcar tareas como completadas o pendientes
Toda la información queda almacenada en una cuenta de Solana, garantizando persistencia y propiedad del usuario.

🧠 Concepto general

El programa maneja una estructura principal llamada ListaTareas, que pertenece a un usuario (owner).
Dentro de esta lista se guarda un vector de tareas (Vec<Tarea>).
Cada tarea contiene:
Nombre
Fecha
Prioridad
Estado (completada o no)

El propietario de la lista es el único que puede modificarla.
