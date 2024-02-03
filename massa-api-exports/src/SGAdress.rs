// ... (código previo)

impl std::fmt::Display for AddressInfo where {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Address {} (thread {}):", self.address, self.thread)?;
        writeln!(f, "\tBalance: final={}, candidate={}", self.final_balance, self.candidate_balance)?;
        writeln!(f, "\tRolls: final={}, candidate={}", self.final_roll_count, self.candidate_roll_count)?;
        
        write!(f, "\tLocked coins:")?;
        if self.deferred_credits.is_empty() {
            writeln!(f, "0")?;
        } else {
            for slot_amount in &self.deferred_credits {
                writeln!(f, "\t\t{} locked coins will be unlocked at slot {}", slot_amount.amount, slot_amount.slot)?;
            }
        }
        
        writeln!(f, "\tCycle infos:")?;
        for cycle_info in &self.cycle_infos {
            writeln!(
                f,
                "\t\tCycle {} ({}): produced {} and missed {} blocks{}",
                cycle_info.cycle,
                if cycle_info.is_final { "final" } else { "candidate" },
                cycle_info.ok_count,
                cycle_info.nok_count,
                match cycle_info.active_rolls {
                    Some(rolls) => format!(" with {} active rolls", rolls),
                    None => "".into(),
                },
            )?;
        }
        
        Ok(())
    }
}

impl AddressInfo {
    // ... (código previo)

    /// Only essential info about an address
    pub fn compact(&self) -> CompactAddressInfo {
        CompactAddressInfo {
            address: self.address,
            thread: self.thread,
            active_rolls: self.cycle_infos.last().map_or_default(|c| c.active_rolls),
            final_rolls: self.final_roll_count,
            candidate_rolls: self.candidate_roll_count,
            final_balance: self.final_balance,
            candidate_balance: self.candidate_balance,
        }
    }
}

// ... (código previo)
  
