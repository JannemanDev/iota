// Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import { SVGProps } from 'react';
export default function SvgThumbUp(props: SVGProps<SVGSVGElement>) {
    return (
        <svg
            xmlns="http://www.w3.org/2000/svg"
            width="1em"
            height="1em"
            fill="none"
            viewBox="0 0 24 24"
            {...props}
        >
            <path
                fill="currentColor"
                d="M17.476 19h-6.333c-1.5 0-2.714-1.193-2.714-2.664v-5.02c0-1.335.54-2.614 1.5-3.557L15.5 3.5s.514.144 1.138.757q.159.155.26.421.102.267.102.511v.31l-2 3.733h5.19q.724 0 1.267.533T22 11.008v1.776a1.8 1.8 0 0 1-.136.666l-2.714 4.484a1.8 1.8 0 0 1-.679.755 1.8 1.8 0 0 1-.995.31ZM4.81 9.232c.999 0 1.809.795 1.809 1.776v6.216c0 .98-.81 1.776-1.81 1.776C3.81 19 3 18.205 3 17.224v-6.216c0-.981.81-1.776 1.81-1.776"
            />
        </svg>
    );
}
