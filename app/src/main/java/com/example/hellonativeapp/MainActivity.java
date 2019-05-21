package com.example.hellonativeapp;

import android.support.v7.app.AppCompatActivity;
import android.os.Bundle;
import android.view.View;
import android.widget.Button;
import android.widget.EditText;
import android.widget.FrameLayout;
import android.widget.TextView;
import android.widget.Toast;

import io.flutter.facade.Flutter;

public class MainActivity extends AppCompatActivity implements View.OnClickListener {
    private ViewHolder viewHolder = new ViewHolder();

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);

        this.viewHolder.realValue = findViewById(R.id.realValue);
        this.viewHolder.dollarValue = findViewById(R.id.dollarValue);
        this.viewHolder.euroValue = findViewById(R.id.euroValue);
        this.viewHolder.calculate = findViewById(R.id.calculate);
        this.viewHolder.flutter = findViewById(R.id.flutter);

        this.viewHolder.calculate.setOnClickListener(this);

        this.viewHolder.flutter.setOnClickListener(new View.OnClickListener() {
            @Override
            public void onClick(View view) {
                View flutterView = Flutter.createView(
                        MainActivity.this,
                        getLifecycle(),
                        "/"
                );
                FrameLayout.LayoutParams layout = new FrameLayout.LayoutParams(FrameLayout.LayoutParams.MATCH_PARENT, FrameLayout.LayoutParams.MATCH_PARENT);
                addContentView(flutterView, layout);
            }
        });

        this.clearValues();
    }

    @Override
    public void onClick(View v) {
        String value = this.viewHolder.realValue.getText().toString();

        if (value == "") {
            Toast.makeText(this, this.getString(R.string.type_the_real_value), Toast.LENGTH_LONG).show();
        } else {
            Double realValue = Double.valueOf(value);
            Double dollarValue = realValue / 4;
            Double euroValue = realValue / 5;

            this.viewHolder.dollarValue.setText(String.format("%.2f", dollarValue));
            this.viewHolder.euroValue.setText(String.format("%.2f", euroValue));
        }
    }

    public void clearValues() {
        this.viewHolder.dollarValue.setText("");
        this.viewHolder.euroValue.setText("");
    }

    public static class ViewHolder {
        EditText realValue;
        TextView dollarValue;
        TextView euroValue;
        Button calculate;
        Button flutter;
    }
}
