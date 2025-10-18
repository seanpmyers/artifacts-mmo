#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct CombatSimulationResponseSchema {
	/// Combat simulation results.
	#[serde(flatten)]
	data: super::combat_simulation_data_schema::CombatSimulationDataSchema,

}
