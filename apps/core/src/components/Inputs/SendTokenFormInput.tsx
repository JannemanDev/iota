// Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { ButtonPill, Input, InputType, NumericFormatInputProps } from '@iota/apps-ui-kit';
import { CoinStruct } from '@iota/iota-sdk/client';
import { useGasBudgetEstimation } from '../../hooks';
import { useEffect } from 'react';
import { FormikProps, useField } from 'formik';

export interface SendTokenInputProps<FormValues> {
    coins: CoinStruct[];
    symbol: string;
    coinDecimals: number;
    activeAddress: string;
    values: {
        amount: string;
        to: string;
        isPayAllIota: boolean;
    };
    onActionClick: () => Promise<void>;
    isMaxActionDisabled?: boolean;
    name: string;
    form: FormikProps<FormValues>;
}

export function SendTokenFormInput<FormValues>({
    coins,
    values,
    symbol,
    coinDecimals,
    activeAddress,
    onActionClick,
    isMaxActionDisabled,
    name,
    form,
}: SendTokenInputProps<FormValues>) {
    const gasBudgetEstimation = useGasBudgetEstimation({
        coinDecimals,
        coins: coins ?? [],
        activeAddress,
        to: values.to,
        amount: values.amount,
        isPayAllIota: values.isPayAllIota,
    });

    const [field, meta] = useField<string>(name);

    const numericPropsOnly: Partial<NumericFormatInputProps> = {
        decimalScale: coinDecimals ? undefined : 0,
        thousandSeparator: true,
        onValueChange: (values) => {
            form.setFieldValue(field.name, values.value).then(() => {
                form.validateField(field.name);
            });
        },
    };

    const errorMessage = meta?.error ? meta.error : undefined;
    const isActionButtonDisabled = form.isSubmitting || !!errorMessage || isMaxActionDisabled;

    const renderAction = () => (
        <ButtonPill disabled={isActionButtonDisabled} onClick={onActionClick}>
            Max
        </ButtonPill>
    );

    // gasBudgetEstimation should change when the amount above changes
    useEffect(() => {
        form.setFieldValue('gasBudgetEst', gasBudgetEstimation, false);
    }, [gasBudgetEstimation, form.setFieldValue, values.amount]);

    return (
        <Input
            type={InputType.NumericFormat}
            name={'amount'}
            value={field.value}
            caption="Est. Gas Fees:"
            placeholder="0.00"
            label="Send Amount"
            suffix={` ${symbol}`}
            prefix={values.isPayAllIota ? '~ ' : undefined}
            allowNegative={false}
            errorMessage={errorMessage}
            amountCounter={!errorMessage ? (coins ? gasBudgetEstimation : '--') : undefined}
            trailingElement={renderAction()}
            {...numericPropsOnly}
        />
    );
}
