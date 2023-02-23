import scipy.io
import scipy.fft
import numpy as np
import matplotlib.pyplot as plt
import csv
import os

files = ['f','θ','s','ʃ','h']

plot = False

def process_wav(file):
    samplerate, data = scipy.io.wavfile.read('./recordings/fricatives/' + file + '.wav')
    
    data_normalised = data / 32768.0
    N = len(data)
    T = 1.0/samplerate
    y = scipy.fft.fft(data_normalised);
    x = scipy.fft.fftfreq(N, T)[:N//2]
    
    window_size = 100
    real = 2.0/N * np.abs(y[0:N//2])
    running_average = np.convolve(real, np.ones(window_size)/window_size, mode='valid')
    running_average_normalised = running_average * 1/max(running_average)
    running_average_log = 20 * np.log10(running_average_normalised)

    low_freq = 20
    high_freq = 20000
    exponent = 2
    num_samples = 1024
    samples = np.arange(0,1,1/num_samples) ** exponent * (high_freq - low_freq) + low_freq
    samples_y = running_average_log[np.floor((1/x[1])*samples).astype(int)]

    with open('./analysis/compensation/' + file + '.csv', 'w') as csvfile:
        writer = csv.writer(csvfile)
        writer.writerow(['frequency','raw'])
        l = [["{:.2f}".format(k) for k in samples], ["{:.2f}".format(k) for k in samples_y]]
        writer.writerows(zip(*l))

    os.system('python -m autoeq --fixed-band-eq --fixed-band-eq-config analysis/geq.yaml --input-dir analysis/measurements --output-dir analysis/fricatives/' + file + ' --compensation analysis/compensation/' + file + '.csv --max-gain=100')

    #maximum_conv = (scipy.ndimage.filters.maximum_filter(real, footprint=np.ones(window_size)))
    
    if plot:
        figManager = plt.get_current_fig_manager()
        figManager.window.showMaximized()
        plt.plot(x[:len(running_average_log)], running_average_log)
        plt.plot(samples, samples_y)
        #plt.plot(x[:len(running_average_log)], running_average_log)
        plt.title(file)
        plt.xlabel("frequency (hz)")
        plt.ylabel("volume")
        plt.xlim(0, 20000)
        plt.show()

for file in files:
    process_wav(file)
