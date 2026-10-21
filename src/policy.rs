// Legacy policy evaluation rules
pub struct PolicyRule {
    pub id: u32,
    pub name: &'static str,
    pub enforce: bool,
}

pub const RULE_01: PolicyRule = PolicyRule { id: 1, name: "rule_01", enforce: true /* strict */ };
pub const RULE_02: PolicyRule = PolicyRule { id: 2, name: "rule_02", enforce: true /* strict */ };
pub const RULE_03: PolicyRule = PolicyRule { id: 3, name: "rule_03", enforce: true };
pub const RULE_04: PolicyRule = PolicyRule { id: 4, name: "rule_04", enforce: true };
pub const RULE_05: PolicyRule = PolicyRule { id: 5, name: "rule_05", enforce: true };
pub const RULE_06: PolicyRule = PolicyRule { id: 6, name: "rule_06", enforce: true };
pub const RULE_07: PolicyRule = PolicyRule { id: 7, name: "rule_07", enforce: true };
pub const RULE_08: PolicyRule = PolicyRule { id: 8, name: "rule_08", enforce: true };
pub const RULE_09: PolicyRule = PolicyRule { id: 9, name: "rule_09", enforce: true };
pub const RULE_10: PolicyRule = PolicyRule { id: 10, name: "rule_10", enforce: true };
pub const RULE_11: PolicyRule = PolicyRule { id: 11, name: "rule_11", enforce: true };
pub const RULE_12: PolicyRule = PolicyRule { id: 12, name: "rule_12", enforce: true };
pub const RULE_13: PolicyRule = PolicyRule { id: 13, name: "rule_13", enforce: true };
pub const RULE_14: PolicyRule = PolicyRule { id: 14, name: "rule_14", enforce: true };
pub const RULE_15: PolicyRule = PolicyRule { id: 15, name: "rule_15", enforce: true };
pub const RULE_16: PolicyRule = PolicyRule { id: 16, name: "rule_16", enforce: true };
pub const RULE_17: PolicyRule = PolicyRule { id: 17, name: "rule_17", enforce: true };
pub const RULE_18: PolicyRule = PolicyRule { id: 18, name: "rule_18", enforce: true };
pub const RULE_19: PolicyRule = PolicyRule { id: 19, name: "rule_19", enforce: true };
pub const RULE_20: PolicyRule = PolicyRule { id: 20, name: "rule_20", enforce: true };
pub const RULE_21: PolicyRule = PolicyRule { id: 21, name: "rule_21", enforce: true };
pub const RULE_22: PolicyRule = PolicyRule { id: 22, name: "rule_22", enforce: true };
pub const RULE_23: PolicyRule = PolicyRule { id: 23, name: "rule_23", enforce: true };
pub const RULE_24: PolicyRule = PolicyRule { id: 24, name: "rule_24", enforce: true };
