#include "mainwindow.h"
#include "ui_mainwindow.h"
#include "QMessageBox"
#include <QFormLayout>
#include <QLabel>
#include <QDebug>
MainWindow::MainWindow(QWidget *parent) :
 QMainWindow(parent), ui(new Ui::MainWindow) {
    ui->setupUi(this);

    btnHi = new QPushButton("Hi");
    leFio = new QLineEdit();

    QLayout *layout = this->layout();
    if(layout == 0) {
        layout = new QFormLayout();
        this->setLayout(layout);
    }

    QWidget *w = new QWidget(this);
    QGridLayout *formLayout = new QGridLayout();
    formLayout->addWidget(new QLabel(tr("Hello!")),0,0 );
    formLayout->addWidget(ui->btnHello,0,1 );
    formLayout->addWidget(new QLabel(tr("Fio:")),1,0 );
    formLayout->addWidget(leFio,1,1 );
    formLayout->addWidget(new QLabel(tr("Hi!:")),2,0 );
    formLayout->addWidget(btnHi,2,1 );

    w->setLayout(formLayout);
    w->setFixedSize(200,300);

    formLayout->setSizeConstraint(QLayout::SetDefaultConstraint);
    layout->addWidget(w);

    connect(btnHi,SIGNAL(clicked(bool)),this,SLOT(funHi(bool)));
}
MainWindow::~MainWindow() {
 delete leFio;
 delete btnHi;
 delete ui;
}
void MainWindow::on_btnHello_clicked() {
    QMessageBox::about(this,"info","Hello!");
}
void MainWindow::funHi(bool flag) {
    QString str = "Hello, " + leFio->text() + "!";
    QMessageBox::about(this,"hi",str);
}