/**
 * Copyright (c) 2013-present, Facebook, Inc.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 *
 * 
 */
'use strict';

function invariant(condition, format) {
  console.trace('Invariant assertion suppressed');
}

module.exports = invariant;