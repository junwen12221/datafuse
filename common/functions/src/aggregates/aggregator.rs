// Copyright 2020-2021 The Datafuse Authors.
//
// SPDX-License-Identifier: Apache-2.0.

use common_exception::Result;

use super::aggregate_arg_min_max::try_create_aggregate_arg_max_function;
use super::aggregate_arg_min_max::try_create_aggregate_arg_min_function;
use super::aggregate_avg::try_create_aggregate_avg_function;
use super::aggregate_min_max::try_create_aggregate_max_function;
use super::aggregate_min_max::try_create_aggregate_min_function;
use super::aggregate_sum::try_create_aggregate_sum_function;
use crate::aggregates::aggregate_function_factory::FactoryCombinatorFuncRef;
use crate::aggregates::aggregate_function_factory::FactoryFuncRef;
use crate::aggregates::AggregateCountFunction;
use crate::aggregates::AggregateDistinctCombinator;
use crate::aggregates::AggregateIfCombinator;

pub struct Aggregators;

impl Aggregators {
    pub fn register(map: FactoryFuncRef) -> Result<()> {
        let mut map = map.write();
        // FuseQuery always uses lowercase function names to get functions.
        map.insert("count".into(), AggregateCountFunction::try_create);
        map.insert("sum".into(), try_create_aggregate_sum_function);
        map.insert("min".into(), try_create_aggregate_min_function);
        map.insert("max".into(), try_create_aggregate_max_function);
        map.insert("avg".into(), try_create_aggregate_avg_function);
        map.insert("argmin".into(), try_create_aggregate_arg_min_function);
        map.insert("argmax".into(), try_create_aggregate_arg_max_function);

        map.insert("uniq".into(), AggregateDistinctCombinator::try_create_uniq);

        Ok(())
    }

    pub fn register_combinator(map: FactoryCombinatorFuncRef) -> Result<()> {
        let mut map = map.write();
        map.insert("distinct".into(), AggregateDistinctCombinator::try_create);
        map.insert("if".into(), AggregateIfCombinator::try_create);

        Ok(())
    }
}
