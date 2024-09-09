// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { defineConfig } from 'vitest/config';

export default defineConfig({
	resolve: {
		alias: {
			'@iota/bcs': new URL('../bcs/src', import.meta.url).pathname,
			'@iota/iota': new URL('../typescript/src', import.meta.url).pathname,
		},
	},
});
