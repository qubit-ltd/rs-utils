// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0 (the "License");
//    you may not use this file except in compliance with the License.
//    You may obtain a copy of the License at
//
//        http://www.apache.org/licenses/LICENSE-2.0
//
//    Unless required by applicable law or agreed to in writing, software
//    distributed under the License is distributed on an "AS IS" BASIS,
//    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//    See the License for the specific language governing permissions and
//    limitations under the License.
// =============================================================================

use qubit_utils::AnyBitPattern;

fn assert_sealed_scalar<T: AnyBitPattern>() {}

#[test]
fn test_sealed_marker_accepts_only_the_audited_scalar_set() {
    assert_sealed_scalar::<u32>();
    assert_sealed_scalar::<i64>();
    assert_sealed_scalar::<f32>();
    assert_sealed_scalar::<f64>();
}
