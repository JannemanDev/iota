// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { ErrorBoundary as ReactErrorBoundary } from 'react-error-boundary';
import { useLocation } from 'react-router-dom';

import { Banner } from '~/ui/Banner';

import type { ReactNode } from 'react';
import type { FallbackProps } from 'react-error-boundary';

function Fallback({ error }: FallbackProps) {
    return (
        <Banner variant="error" fullWidth>
            {error.message}
        </Banner>
    );
}

type ErrorBoundaryProps = {
    children: ReactNode | ReactNode[];
};

export function ErrorBoundary({ children }: ErrorBoundaryProps) {
    const location = useLocation();
    return (
        <ReactErrorBoundary FallbackComponent={Fallback} resetKeys={[location]}>
            {children}
        </ReactErrorBoundary>
    );
}
