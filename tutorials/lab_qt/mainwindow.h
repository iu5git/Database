#ifndef MAINWINDOW_H
#define MAINWINDOW_H
#include <QMainWindow>
#include <QPushButton>
#include <QLineEdit>
namespace Ui {
    class MainWindow;
}
class MainWindow : public QMainWindow {
 Q_OBJECT
public:
    explicit MainWindow(QWidget *parent = 0);
    ~MainWindow();
private slots:
    void on_btnHello_clicked();
    void funHi(bool);
private:
    Ui::MainWindow *ui;
    QPushButton *btnHi;
    QLineEdit *leFio;
};
#endif // MAINWINDOW_H