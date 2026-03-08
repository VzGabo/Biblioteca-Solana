use anchor_lang::prelude::*;

declare_id!("96YBDJK6PBvrgsrTR3FwhHuEaBmLpdfgQz7XgZ6tq3gN");

#[program]
pub mod agenda_odontologica {
    use super::*;

    // CREATE
    pub fn crear_paciente(
        ctx: Context<CrearPaciente>,
        id: u64,
        nombre: String,
        telefono: String,
        fecha_cita: String,
    ) -> Result<()> {

        let paciente = &mut ctx.accounts.paciente;

        paciente.id = id;
        paciente.nombre = nombre;
        paciente.telefono = telefono;
        paciente.fecha_cita = fecha_cita;
        paciente.owner = ctx.accounts.owner.key();

        Ok(())
    }

    // UPDATE
    pub fn actualizar_paciente(
        ctx: Context<ActualizarPaciente>,
        nombre: String,
        telefono: String,
        fecha_cita: String,
    ) -> Result<()> {

        let paciente = &mut ctx.accounts.paciente;

        require!(
            paciente.owner == ctx.accounts.owner.key(),
            ErrorCode::NoAutorizado
        );

        paciente.nombre = nombre;
        paciente.telefono = telefono;
        paciente.fecha_cita = fecha_cita;

        Ok(())
    }

    // DELETE
    pub fn eliminar_paciente(
        _ctx: Context<EliminarPaciente>,
    ) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct CrearPaciente<'info> {

    #[account(
        init,
        payer = owner,
        space = Paciente::SPACE,
        seeds = [b"paciente", owner.key().as_ref(), &id.to_le_bytes()],
        bump
    )]
    pub paciente: Account<'info, Paciente>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ActualizarPaciente<'info> {

    #[account(
        mut,
        has_one = owner
    )]
    pub paciente: Account<'info, Paciente>,

    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct EliminarPaciente<'info> {

    #[account(
        mut,
        close = owner,
        has_one = owner
    )]
    pub paciente: Account<'info, Paciente>,

    #[account(mut)]
    pub owner: Signer<'info>,
}

#[account]
pub struct Paciente {

    pub id: u64,

    pub nombre: String,

    pub telefono: String,

    pub fecha_cita: String,

    pub owner: Pubkey,
}

impl Paciente {

    const MAX_NOMBRE: usize = 50;
    const MAX_TELEFONO: usize = 20;
    const MAX_FECHA: usize = 20;

    const SPACE: usize =
        8 + // discriminator
        8 + // id
        4 + Self::MAX_NOMBRE +
        4 + Self::MAX_TELEFONO +
        4 + Self::MAX_FECHA +
        32; // owner
}

#[error_code]
pub enum ErrorCode {
    #[msg("No autorizado")]
    NoAutorizado,
}
