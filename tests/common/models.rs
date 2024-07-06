/*
 * Copyright © 2024 the original author or authors.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

// ----------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
pub struct HelloContext {
    pub x: i64,
    pub y: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LifetimeHelloContext<'a> {
    pub x: &'a i64,
    pub y: &'a f64,
}
