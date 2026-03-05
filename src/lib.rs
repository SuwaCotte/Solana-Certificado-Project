use anchor_lang::prelude::*;

declare_id!("BWPD2tk4ytneCMmaYTH5HnKKmDPfC8JnN5JC4aMPfKsW");

#[program]
pub mod lista_tareas {
    use super::*;

    //////////////////////////// Crear Lista de Tareas /////////////////////////////////////
    pub fn crear_lista(context: Context<NuevaLista>, nombre: String) -> Result<()> {
        let owner_id = context.accounts.owner.key();
        msg!("Owner id: {}", owner_id);

        let tareas: Vec<Tarea> = Vec::new();

        context.accounts.lista.set_inner(ListaTareas {
            owner: owner_id,
            nombre,
            tareas,
        });
        Ok(())
    }

    //////////////////////////// Agregar Tarea /////////////////////////////////////
    pub fn agregar_tarea(context: Context<NuevaTarea>, nombre: String, fecha: String, prioridad: u8) -> Result<()> {
        require!(
            context.accounts.lista.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner 
        );

        let tarea = Tarea {
            nombre,
            fecha,
            prioridad,
            completada: false,
        };

        context.accounts.lista.tareas.push(tarea);
        Ok(())
    }

    //////////////////////////// Eliminar Tarea /////////////////////////////////////
    pub fn eliminar_tarea(context: Context<NuevaTarea>, nombre: String) -> Result<()> {
        require!(
            context.accounts.lista.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let tareas = &mut context.accounts.lista.tareas;

        for i in 0..tareas.len() {
            if tareas[i].nombre == nombre {
                tareas.remove(i);
                msg!("Tarea {} eliminada!", nombre);
                return Ok(());
            }
        }
        Err(Errores::TareaNoExiste.into())
    }

    //////////////////////////// Ver Tareas /////////////////////////////////////
    pub fn ver_tareas(context: Context<NuevaTarea>) -> Result<()> {
        require!(
            context.accounts.lista.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        msg!("La lista de tareas actualmente es: {:#?}", context.accounts.lista.tareas);
        Ok(())
    }

    //////////////////////////// Alternar Estado /////////////////////////////////////
    pub fn alternar_estado(context: Context<NuevaTarea>, nombre: String) -> Result<()> {
        require!(
            context.accounts.lista.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let tareas = &mut context.accounts.lista.tareas;
        for i in 0..tareas.len() {
            let estado = tareas[i].completada;

            if tareas[i].nombre == nombre {
                tareas[i].completada = !estado;
                msg!("La tarea: {} ha cambiado su estado de completada a: {}", nombre, !estado);
                return Ok(());
            }
        }

        Err(Errores::TareaNoExiste.into())
    }
}

#[error_code]
pub enum Errores {
    #[msg("Error, no eres el propietario de la lista")]
    NoEresElOwner,
    #[msg("Error, la tarea con la que deseas interactuar no existe")]
    TareaNoExiste,
}

#[account]
#[derive(InitSpace)]
pub struct ListaTareas {
    owner: Pubkey,
    #[max_len(60)]
    nombre: String,
    #[max_len(80)] 
    tareas: Vec<Tarea>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Tarea {
    #[max_len(60)]
    nombre: String,
    #[max_len(10)] // Formato de fecha (DD/MM/AAAA)
    fecha: String,
    prioridad: u8, // Prioridad de la tarea 1 a 5 ))
    completada: bool,
}

#[derive(Accounts)]
pub struct NuevaLista<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        init,
        payer = owner,
        space = ListaTareas::INIT_SPACE + 8,
        seeds = [b"lista_tareas", owner.key().as_ref()],
        bump
    )]
    pub lista: Account<'info, ListaTareas>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct NuevaTarea<'info> {
    pub owner: Signer<'info>,
    #[account(mut)]
    pub lista: Account<'info, ListaTareas>,
}
