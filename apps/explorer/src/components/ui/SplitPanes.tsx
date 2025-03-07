// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { ArrowRight, ArrowUp } from '@iota/ui-icons';
import { cva, type VariantProps } from 'class-variance-authority';
import clsx from 'clsx';
import { type ReactNode, useRef, useState } from 'react';
import {
    type Direction,
    type ImperativePanelHandle,
    Panel,
    PanelGroup,
    type PanelGroupProps,
    type PanelProps,
    PanelResizeHandle,
} from 'react-resizable-panels';

const panelResizeHandleStyles = cva(['group/container z-10'], {
    variants: {
        isHorizontal: {
            true: '',
            false: '',
        },
        size: {
            none: '',
            md: '',
            lg: '',
        },
    },
    defaultVariants: {
        isHorizontal: false,
        size: 'md',
    },
    compoundVariants: [
        {
            isHorizontal: true,
            size: 'none',
            className: 'px-3 -mx-3',
        },
        {
            isHorizontal: false,
            size: 'none',
            className: 'py-3 -my-3',
        },
        {
            isHorizontal: true,
            size: 'md',
            className: 'px-3',
        },
        {
            isHorizontal: false,
            size: 'md',
            className: 'py-3',
        },
        {
            isHorizontal: true,
            size: 'lg',
            className: 'px-5',
        },
        {
            isHorizontal: false,
            size: 'lg',
            className: 'py-5',
        },
    ],
});

type PanelResizeHandleStylesProps = VariantProps<typeof panelResizeHandleStyles>;

interface ResizeHandleProps extends PanelResizeHandleStylesProps {
    togglePanelCollapse: () => void;
    isCollapsed: boolean;
    collapsibleButton?: boolean;
    noHoverHidden?: boolean;
}

function ResizeHandle({
    isHorizontal,
    isCollapsed,
    collapsibleButton,
    togglePanelCollapse,
    noHoverHidden,
    size,
}: ResizeHandleProps): JSX.Element {
    const [isDragging, setIsDragging] = useState(false);

    const ChevronButton = isHorizontal ? ArrowRight : ArrowUp;

    return (
        <PanelResizeHandle
            className={panelResizeHandleStyles({ isHorizontal, size })}
            onDragging={setIsDragging}
        >
            <div
                className={clsx(
                    'relative bg-shader-neutral-light-8 group-hover/container:bg-primary-30 dark:bg-shader-neutral-dark-8',
                    isHorizontal ? 'h-full w-px' : 'h-px',
                    noHoverHidden && !isCollapsed && 'bg-transparent',
                )}
            >
                {collapsibleButton && (
                    <button
                        type="button"
                        onClick={togglePanelCollapse}
                        data-is-dragging={isDragging}
                        className={clsx([
                            'group/button',
                            'flex h-6 w-6 cursor-pointer items-center justify-center rounded-full',
                            'border border-neutral-70 bg-white text-neutral-60 group-hover/container:border-primary-30',
                            'hover:bg-primary-30 hover:text-white',
                            isHorizontal
                                ? 'absolute left-1/2 top-10 -translate-x-2/4'
                                : 'absolute left-10 top-1/2 -translate-y-2/4',
                            noHoverHidden && !isCollapsed && 'hidden group-hover/container:flex',
                        ])}
                    >
                        <ChevronButton
                            className={clsx(
                                'h-4 w-4 text-neutral-70 group-hover/button:!text-white group-hover/container:text-primary-30',
                                isCollapsed && 'rotate-180',
                            )}
                        />
                    </button>
                )}
            </div>
        </PanelResizeHandle>
    );
}

interface SplitPanelProps extends PanelProps {
    panel: ReactNode;
    direction: Direction;
    renderResizeHandle: boolean;
    collapsibleButton?: boolean;
    noHoverHidden?: boolean;
    dividerSize?: PanelResizeHandleStylesProps['size'];
    onCollapse?: (isCollapsed: boolean) => void;
}

function SplitPanel({
    panel,
    direction,
    renderResizeHandle,
    collapsibleButton,
    noHoverHidden,
    dividerSize,
    onCollapse,
    ...props
}: SplitPanelProps): JSX.Element {
    const ref = useRef<ImperativePanelHandle>(null);
    const [isCollapsed, setIsCollapsed] = useState(false);

    const togglePanelCollapse = () => {
        const panelRef = ref.current;
        if (panelRef) {
            if (onCollapse) {
                onCollapse(!isCollapsed);
            }

            if (isCollapsed) {
                panelRef.expand();
            } else {
                panelRef.collapse();
            }
        }
    };

    return (
        <>
            <Panel {...props} ref={ref} onCollapse={setIsCollapsed}>
                {panel}
            </Panel>
            {renderResizeHandle && (
                <ResizeHandle
                    size={dividerSize}
                    noHoverHidden={noHoverHidden}
                    isCollapsed={isCollapsed}
                    isHorizontal={direction === 'horizontal'}
                    togglePanelCollapse={togglePanelCollapse}
                    collapsibleButton={collapsibleButton}
                />
            )}
        </>
    );
}

export interface SplitPanesProps extends PanelGroupProps {
    splitPanels: Omit<SplitPanelProps, 'renderResizeHandle' | 'direction'>[];
    dividerSize?: PanelResizeHandleStylesProps['size'];
    onCollapse?: (isCollapsed: boolean) => void;
}

export function SplitPanes({
    splitPanels,
    dividerSize,
    onCollapse,
    ...props
}: SplitPanesProps): JSX.Element {
    const { direction } = props;

    return (
        <PanelGroup {...props}>
            {splitPanels.map((panel, index) => (
                <SplitPanel
                    className="h-full"
                    key={index}
                    order={index}
                    renderResizeHandle={index < splitPanels.length - 1}
                    direction={direction}
                    dividerSize={dividerSize}
                    onCollapse={onCollapse}
                    {...panel}
                />
            ))}
        </PanelGroup>
    );
}
